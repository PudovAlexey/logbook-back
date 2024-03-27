
pub mod service {
    extern crate image;
    use crate::images::model::{
        CreateImageQuery, Image,CreateImage, CreateAvatarQuery
    };

    use diesel::{
        prelude::*,
        r2d2::{ConnectionManager, PooledConnection}, ExpressionMethods, PgConnection, RunQueryDsl
    };

    type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;
    
    pub struct ImagesTable {
        connection: PooledPg
    }
    
    
    impl ImagesTable {
        pub fn new(connection: PooledPg) -> ImagesTable {
            ImagesTable { connection }
        }
        
        pub fn set_image(& mut self, params: CreateImage) -> Result<i32, diesel::result::Error> {
            use crate::schema::image::dsl::*;
            let CreateImage { path: pathname, filename: file, created_at: created, updated_at: updated, ..} = params;

           let create_image = diesel::insert_into(image).values((
            path.eq(pathname),
            filename.eq(file),
            created_at.eq(created),
            updated_at.eq(updated),
        ))
            .returning(id)
            .get_result(&mut self.connection);

            create_image
        }

        pub fn set_avatar(& mut self, params: CreateAvatarQuery) -> Result<i32, diesel::result::Error> {
            use crate::schema::avatar::dsl::*;
            let CreateAvatarQuery {image_data, user_id: spec_user_id} = params;

            let create_image_result = self.set_image(CreateImage::from(image_data));

            
            if create_image_result.is_ok() {
             let create_avatar = diesel::insert_into(avatar).values((
                image_id.eq(create_image_result.unwrap()),
                user_id.eq(spec_user_id)
             ))
             .returning(id)
             .get_result(&mut self.connection);

             create_avatar

            } else {
                let err = create_image_result.unwrap_err();

                Err(err)
            }
            
        }
    }
}
pub mod service {
    extern crate image;

    use crate::images::model::{AvatarInfo, CreateAvatarQuery, CreateImage, LogImageInfo};
    

    use diesel::{
        prelude::*,
        r2d2::{ConnectionManager, PooledConnection},
        ExpressionMethods, PgConnection, RunQueryDsl,
    };

    type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

    pub struct ImagesTable {
        connection: PooledPg,
    }

    impl ImagesTable {
        pub fn new(connection: PooledPg) -> ImagesTable {
            ImagesTable { connection }
        }

        pub fn set_image(&mut self, params: CreateImage) -> Result<i32, diesel::result::Error> {
            use crate::schema::image::dsl::*;
            let CreateImage {
                path: pathname,
                filename: file,
                created_at: created,
                updated_at: updated,
                ..
            } = params;

            let create_image = diesel::insert_into(image)
                .values((
                    path.eq(pathname),
                    filename.eq(file),
                    created_at.eq(created),
                    updated_at.eq(updated),
                ))
                .returning(id)
                .get_result(&mut self.connection);

            create_image
        }

        pub fn set_avatar(
            &mut self,
            params: CreateAvatarQuery,
        ) -> Result<i32, diesel::result::Error> {
            use crate::schema::avatar::dsl::*;
            let CreateAvatarQuery {
                image_data,
                user_id: spec_user_id,
            } = params;

            let create_image_result = self.set_image(CreateImage::from(image_data));

            if create_image_result.is_ok() {
                let create_avatar = diesel::insert_into(avatar)
                    .values((
                        image_id.eq(create_image_result.unwrap()),
                        user_id.eq(spec_user_id),
                    ))
                    .returning(id)
                    .get_result(&mut self.connection);

                create_avatar
            } else {
                let err = create_image_result.unwrap_err();

                Err(err)
            }
        }

        pub fn get_avatar_data(&mut self, avatar_id: i32)-> Result<AvatarInfo, diesel::result::Error> {
            use crate::schema::avatar;
            use crate::schema::image;

            let avatar_data: Result<AvatarInfo, diesel::result::Error> = avatar::table
                .inner_join(image::table.on(image::columns::id.eq(avatar::columns::image_id)))
                .filter(avatar::columns::id.eq(avatar_id))
                .select(AvatarInfo::as_select())
                .first::<AvatarInfo>(&mut self.connection);

            avatar_data
        }

        pub fn get_log_image_data(&mut self, logbook_id: i32)-> Result<LogImageInfo, diesel::result::Error> {
            use crate::schema::log_image;
            use crate::schema::image;

            let log_image_data: Result<LogImageInfo, diesel::result::Error> = log_image::table
                .inner_join(image::table.on(image::columns::id.eq(log_image::columns::image_id)))
                .filter(log_image::columns::id.eq(logbook_id))
                .select(LogImageInfo::as_select())
                .first::<LogImageInfo>(&mut self.connection);

            log_image_data
        }
    }
}

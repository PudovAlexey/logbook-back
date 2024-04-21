FROM rust:1.67

WORKDIR /app
EXPOSE 3003

COPY . .

RUN make release

EXPOSE 8081


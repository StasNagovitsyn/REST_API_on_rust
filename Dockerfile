FROM rust:1.67

COPY . .
WORKDIR .


EXPOSE 127.0.0.1:3000

CMD ["cargo", "run"]
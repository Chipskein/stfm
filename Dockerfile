FROM rust:1.80
WORKDIR /usr/src/stfm
COPY . .
RUN cargo install --path .
#CMD ["tail","-f","/dev/null"]
CMD ["stfm"]
#docker build -t stfm .
#docker run -it --rm --name stfm stfm


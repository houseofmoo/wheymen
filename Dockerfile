# EXPECT HOST MACHINE TO HAVE NPM 8.15.0, NODE 18.7.0, and CARGO 1.63.
# .env file will have to be manually created with expected values before 
# server can be compiled.
# Host must have run 'rustup target add x86_64-unknown-linux-gnu' to compile for linux (if on windows)
# problem: compiling on windows for linux is not as straight forward as i hoped

#base linux image
FROM alpine

# install surrealdb
# curl -sSf https://install.surrealdb.com | sh

# compile client code and copy to ./dist directory
# cp ./server/dist ./app/dist

# compile server code and copy
# cargo build --release --manifest-path .\server\Cargo.toml
# cp ./server/target/release/wheymen_surreal.exe ../app/server.exe

# add export port 

# run surrealdb commnad (listens on localhost:8000)
# surreal start --user root --pass root file://C:\Users\yourg\Desktop\wheymen\db

# run server command
# ./serer.exe
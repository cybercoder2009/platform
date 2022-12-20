cargo build --release \
&& scp -r ./target/release/server ./target/release/fonts_checker ./nginx ./config.toml ./run.sh root@45.79.116.50:~/server
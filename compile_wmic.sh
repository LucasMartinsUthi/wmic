docker build -t monsta/wmic:latest .
docker run -it --name monsta_wmic monsta/wmic:latest


docker cp monsta_wmic:wmic/target/release/wmic .
docker stop monsta_wmic
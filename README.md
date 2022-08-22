# Compilando WMIC usando Docker

Buildando a imagem

`docker build -t monsta/wmic:latest .`

Criando o container, ao executar o `run` o `cargo build --release` já é feito automaticamente

`docker run -it --name monsta_wmic monsta/wmic:latest`

Extraindo o executavel

`
    docker cp monsta_wmic:wmic/target/release/wmic .
    docker stop monsta_wmic
`

## Comandos uteis

```
cargo run -- //127.0.0.1 "select Name from Win32_Service"
./wmic -U usuario%senha //192.168.65.2 "select Name from Win32_Service"
```

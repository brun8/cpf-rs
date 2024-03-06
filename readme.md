# cpf.rs

gerador de cpf na linha de comando

## instalação

se já tiver 'cargo' instalado e repositório clonado
```sh
cargo install --path <path-do-repositorio>
```

## uso

para gerar um cpf
```sh
cpf
```

para gerar 10 cpfs
```sh
cpf -n 10
```

para gerar um cpf com pontuação
```sh
cpf -p
```

para gerar um cpf de uma região específica
```sh
cpf -r 1
```

para gerar vários cpfs de uma região específica com pontuação
```sh
cpf -n 10 -r 8 -p
```


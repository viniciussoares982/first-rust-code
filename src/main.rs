fn main() {
    println!("Olá, mundo!");
}

// comandos CARGO

// cargo build - ele irá montar o seu programa, e será pronto para ser executado.
// o .exe vai estar dentro da pasta /target/debug/%arquivo%.exe

// cargo run - Após isso, se não for necessario buildar o programa novamente, você pode rodar esse comando para executa-lo!

// cargo check - ele não irá gerar um executavel, mas porque? Ele e muito mais rapido que o cargo build, para você checar se existe algum problema ao compilar.

// cargo build --release - este comando irá otimizar o seu codigo, para o lançamento final, quando seu programa estiver pronto.
// ele será salvo no target/release
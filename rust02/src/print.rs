pub fn run() {
    println!("Hello from print.rs!");
    println!("Printing a number with formatting: {}, and another one {}", 190, 90);
    println!("{0} I can set numbers for the params! {2} {1}", "Cero", "Uno", "Dos");
    println!("{canal}, en Rust puedes poner nombres a los parametros tambien! {name}", canal="BettaTech", name="Que Lo Sepas");
    println!("Ademas tambien es muy facil convertir numeros a binario, mira! {} -> {:b}", 12, 12);
    println!("Printing a tuple: {:?}", ("uno", true, 12));
}

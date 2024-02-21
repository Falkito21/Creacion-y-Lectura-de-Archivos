// Indicamos que utilizaremos el modulo File (nativo de rust)
use std::fs::File; 
//llamamos al module que necesitamos de io
use std::io::{BufReader, Read, Write};
//sino podemos llamar directamente a todo
// use std::io::prelude::*;
use std::fs::OpenOptions;//modulo para agrigar contenido al archivo en vez de sobre escribirlo 
use std::io::BufRead;

// Todo lo indicado despues de -> hace referencia a lo que queremos que retorne la funcion 
fn main() -> std::io::Result<()>{ // retornara Result del paquete io 
    //? const CONTENIDO: &str = "Hola como estan? Mi nombre es Juan Asensio Falco y tengo 23 anios. Soy programador junior que aspira un mayor senioriti. Que tenga un lindo dia! Hasta luego.";
    const CONTENIDO: &str = "\nEste es mi nuevo contenido en el archivo txt. Espero funcione como debe. chaoo!";

    //la referencia de su creacion lo vamos a guardar en una variable para para luego modificar su contenido
    //usamos create para crear nuestro archivo ejemplo.txt
    //? let mut archivo = File::create("sobreMi.txt")?; // El ? indica que si crea el archivo, el programa continue con un Result de tipo io

    let mut archivo = OpenOptions::new()
                                    .append(true)//indicamos que queremos abrir el archivo para poder agregar mas contenido, si ya existe
                                    .create(true)//sino existe el archivo, le indicamos que lo cree
                                    .open("sobreMi.txt")?;
    // el contenido que queramos que tenga el archivo lo tendremos que pasar a bytes (ES NECESARIO CONVERTIRLO A BYTES)
    archivo.write_all(CONTENIDO.as_bytes())?;
    Ok(()) //Es el retorno que espera el main
}
 
fn leer_poema() -> std::io::Result<()>{
    //el metodo open abre el archivo pero en modo read-only
    //? let mut archivo = File::open("poema.txt")?; //guardamos en archivo el contenenido que del archivo poema.txt que previamente abrimos
    //? let mut contenido = String::new();
    //Lee el contendio que hay en archivo y lo coloca en un string (contenido)
    //el metodo read_to_string recibe como parametro la referencia de la variable donde vamos al almacenar el contenido del archivo 
    //? archivo.read_to_string(&mut contenido)?;

    let archivo = File::open("poema.txt")?;
    // la ventaja de hacerlo con el buffer es que no es necesario definir a la variable 'archivo' como mut
    let mut buf_reader = BufReader::new(archivo);
    let mut contenido = String::new();

    buf_reader.read_to_string(&mut contenido);

    println!("Contenido del Archivo");
    println!("{}", contenido);

    Ok(())
}

fn leer_archivo_linea_por_linea() -> std::io::Result<()>{
    //guardamos en 'archivo' el contendio de 'poema.txt' en modo read-only
    let archivo = File::open("poema.txt")?;
    //creamos un buffer con el archivo
    let buf_reader = BufReader::new(archivo);

    println!("Contenido del Archivo!");

    //recorremos el contenido de buf_reader linea por linea 
    for linea in buf_reader.lines(){
        //es necesario establecer el unwrap(), en caso de que la linea se 'none' entonces el programa dara un error que sera controlado por el mismo
        println!("{}", linea.unwrap());
    }
    Ok(())
}
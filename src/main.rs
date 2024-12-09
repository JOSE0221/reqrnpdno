use reqrnpdno::{extractora, parameters::Parametros, utilidades};

fn main() {
    // Aquí se define la ruta en donde se guardarán los datos.
    // En este caso es un directorio llamado "30-nov-2021" que aún no existe pero que crearemos en el directorio "salida".
    // Para crearlo usamos la función de ayuda "crear_directorio()" para la cual el primer parámetro es la ruta objetivo y el segundo parámetro es el nombre del directorio a crear.
    let ruta_base = "./salida/";
    let nombre_directorio = "30-nov-2021";

    // Verificar si la ruta base existe
    if !std::path::Path::new(ruta_base).exists() {
        std::fs::create_dir(ruta_base).expect("No se pudo crear el directorio base 'salida'");
    }

    let ruta_salida = utilidades::crear_directorio(ruta_base, nombre_directorio).unwrap();
    println!("Directorio creado en: {:?}", ruta_salida);

    // Aquí creamos la estructura de los parámetros necesarios para realizar la petición.
    // Esta estructura es la que se necesita modificar si quieres aplicar algún filtro.
    // Si no quieres filtrar no es necesario modificar la estructura.
    let parametros = Parametros::new();

    // Para este caso no modificaremos la estructura con los parametros.

    // Por último, utilizamos la función de alto nivel "extraer_todo_iterando" para obtener nuestros datos.
    extractora::extraer_todo_iterando(&parametros, &ruta_salida).unwrap();
}

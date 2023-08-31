#[derive(Debug, Clone)]
pub struct Caso {
    unic_id: i32,
    case_id: String,
    activity: String,
    start_date: String,
    end_date: String,
    agent_position: String,
    customer_id: String,
    product: String,
    service_type: String,
    resource: String
}


fn main() {
    //Creamos una lista de casos    
    let mut casos: Vec<Caso> = Vec::new();
    //leemos el CSV
    let mut rdr = csv::Reader::from_path("data/casos.csv").unwrap();
    let mut i = 0;
    //iteramos sobre el CSV
    for result in rdr.records() {
        //obtenemos el record
        let record = result.unwrap();
        //creamos un caso
        let caso = Caso {
            unic_id: i,
            case_id: record[0].to_string(),
            activity: record[1].to_string(),
            start_date: record[2].to_string(),
            end_date: record[3].to_string(),
            agent_position: record[4].to_string(),
            customer_id: record[5].to_string(),
            product: record[6].to_string(),
            service_type: record[7].to_string(),
            resource: record[8].to_string()
        };
        //lo agregamos a la lista
        casos.push(caso);
        i += 1;
    }

    //Creamos una lista de listas, donde cada lista es un tipo de caso (caso 1, caso 2 y así)
    let mut lista_de_listas_de_casos: Vec<Vec<Caso>> = Vec::new();

    //Recorremos la lista de casos
    for i in 0..casos.len() {
        if lista_de_listas_de_casos.len() > 0 {
            if lista_de_listas_de_casos[lista_de_listas_de_casos.len() - 1][0].case_id == casos[i].case_id {
                continue;
            }
        }
        //Creamos la lista de casos
        let mut lista_de_casos: Vec<Caso> = Vec::new();
        //Recorremos la lista de casos 
        for j in i..casos.len(){
            //Si el caso i es igual al caso j
            if casos[i].case_id == casos[j].case_id {
                lista_de_casos.push(casos[j].clone());
            }
        }
        lista_de_listas_de_casos.push(lista_de_casos);
    }
    println!("Llegamos hasta aquí");
    
    // //Imprimimos la lista de listas
    // for lista_de_casos in lista_de_listas_de_casos {
    //     for caso in lista_de_casos {
    //         print!("[{}]", caso.case_id);
    //     }
    //     println!("");
    // }

    let mut lista_buena: Vec<Caso> = Vec::new();
    let mut lista_mala: Vec<Caso> = Vec::new();
    
    //LISTA MALA: TODO LO QUE SOLO TENGA UNA ACTIVIDAD, O QUE TODAS SUS ACTIVIDADES SEAN INBOUND CALL
    //Recorremos todas las listas de casos
    for lista_de_casos in lista_de_listas_de_casos {
        //Recorremos cada caso de cada lista
        let mut contador = 0; //Si algo no es Inbound Call, la lista es buena.
        for caso in lista_de_casos.clone() {
            if caso.activity != "Inbound Call" {
                contador += 1;
                break;
            }
        }
        //IF contador != 0 Agregamos todos los elementos de la lista de casos a la lista buena
        if contador != 0 {
            for caso in lista_de_casos.clone() {
                lista_buena.push(caso);
            }
        }
        else{
            for caso in lista_de_casos.clone() {
                lista_mala.push(caso);
            }
        }
    }

    //Imprimimos la lista mala
    for caso in lista_mala {
        println!("Caso: {}", caso.case_id);
    }
    //Imprimimos la lista buena
    println!("Lista buena");
    for caso in lista_buena {
        println!("Caso: {} | Activity: {}", caso.case_id, caso.activity);
    }

}

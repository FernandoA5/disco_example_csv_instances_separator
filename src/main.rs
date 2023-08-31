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

    //Empecemos con la lista mala, aquí pondremos todos los casos que solo tengan una actividad
    for lista_de_casos in lista_de_listas_de_casos.clone() {
        if lista_de_casos.len() == 1 {
            lista_mala.push(lista_de_casos[0].clone());
        } else {
            //Aquí debemos seguir filtrando:
            //Temporalmente daremos por buenos aquellos que contengan por lo menos una Handle activity
            //Recorremos la lista de listas de casos 
            for lista_de_casos in lista_de_listas_de_casos.clone() {
                //Recorremos la lista de casos
                let mut valida = false;
                for caso in lista_de_casos.clone() {
                    //Si el caso es una handle activity
                    if caso.activity.contains("Handle") {
                        //Damos la lista por buena
                        valida = true;
                        break;
                    }
                }

                //Si la lista es valida, la agregamos a la lista buena
                if valida {
                    for caso in lista_de_casos {
                        //Revisamos el el unique_id no se encuentre ya en la lista buena
                        for caso_bueno in lista_buena.clone() {
                            if caso_bueno.unic_id == caso.unic_id {
                                continue;
                            }
                        }
                        lista_buena.push(caso);
                    }
                } else {
                    //Si no, la agregamos a la lista mala
                    for caso in lista_de_casos {
                        //Revismos que el unique_id no se encuentre ya en la lista mala
                        for caso_malo in lista_mala.clone() {
                            if caso_malo.unic_id == caso.unic_id {
                                continue;
                            }
                        }
                        lista_mala.push(caso);
                    }
                }
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

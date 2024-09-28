# Setup

Instalar [rust-lang1.81.0](https://www.rust-lang.org/es/tools/install)

Verificar la instalación correcta:

´rustc --version´

## Preparar entorno: 

Rust nos provee `cargo`, un gestor de dependencias propio que nos permite estandarizar las dependeicas y sus versiones. 
Para levantar el proyecto, simplemente corremos este comando (estando dentro de `./app`):

`cargo run`

Los tiempos de compilación de rust no son los más rápidos, por lo que la primera vez tardará unos minutos. Luego, a menos
se haya realizado algún cambio en el código, tardará menos ya que el programa estará cacheado.

# Uso del Servicio

El propósito de este servicio es devolver posibles ubicaciones (a modo de recomendaciòn) a partir de una ubicación y un radio máximo 
sobre el que buscar. 

## Endpoints

Method: `GET`
URL: `/api/personas_vulnerables`
Descripción: a partir de una ubicación y un radio, devuelve un listado con recomendaciones de personas a las que realizar una donación.
Params: 
  * `calle`: `string`
  * `altura`: `string`
  * `provincia`: `string / null`
  * `radio_max`: `float`
Response: 
  ```json
  {
    "nombre": "string",
    "apellido": "string",
    "direccion": {
      "direccion": {
        "calle": "string",
        "altura": int,
        "provincia": "string",
      },
      coordenadas: {
        "latitud": float,
        "longitud": float
      },
      "cantidad_recomendada": int
    },
  }
  ```

> Aclararión: la cantidad recomendada representa la persona vulnerable + sus hijos.

Método: `POST`
URL: `/api/personas_vulnerables`
Descripción: a partir de una persona, la persiste en la base de datos
Body: 
  ```json
  {
    "personas": [*(info) 
      {
        "nombre": "string",
        "apellido": "string",
        "direccion": {
          "calle": "string",
          "altura": int,
          "provincia": "string",
        }
        hijos: [(info)]
      }
    ]
  }
  ````
Respuesta: 
  ```json
  {
    [
      "nombre": "string",
      "apellido": "string",
      "direccion": {
        "direccion": {
          "calle": "string",
          "altura": int,
          "provincia": "string",
        },
        "coordenadas": {
          "latitud": float,
          "longitud": float
        },
      },
    ]
  }
  ```

Method: `GET`
URL: `/api/heladeras`
Descripción: a partir de una ubicación y un radio, devuelve un listado con recomendaciones de heladeras a laa que realizar una donación.
> Tambièn es posible enviar un stock mínimo para filtrar heladeras.
Params:
  * `calle`: `string`
  * `altura`: `string`
  * `provincia`: `string` / `null`
  * `radio_max`: `float`
  * `stock_minimo`: `int` / `null`
Response: 
  ```json
  {
    "direccion": {
      "direccion": {
        "calle": "string",
        "altura": int,
        "provincia": "string",
      },
      "coordenadas": {
        "latitud": float,
        "longitud": float
      },
      "cantidad_viandas": int
    },
  }
  ```

Método: `POST`
URL: `/api/heladeras`
Descripción: a partir de una heladera, la persiste en la base de datos
Body: 
  ```json
  {
    "heladeras": [*(info) 
      {
        "cantidad_viandas": int,
        "direccion": {
          "calle": "string",
          "altura": int,
          "provincia": "string",
        }
      }
    ]
  }
  ````
Respuesta: 
  ```json
  {
    [
      "direccion": {
        "direccion": {
          "calle": "string",
          "altura": int,
          "provincia": "string",
        },
        "coordenadas": {
          "latitud": float,
          "longitud": float
        },
      },
      "cantidad_viandas": int
    ]
  }
  ```

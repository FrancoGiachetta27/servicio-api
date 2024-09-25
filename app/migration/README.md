# Running Migrator CLI

- Generate a new migration file
    ```sh
    cargo run -- generate MIGRATION_NAME
    ```
- Apply all pending migrations
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -- status
    ```

# Recomendación de heladera

Este servicio permite manejar un API que, a partir de una serie de parámetros ingresados, devuelve

una recomendación basada en la ubicación de la heladera sugerida para la donación.


## ¿Cómo funciona?

La funcion get_recomendacion maneja el endpoint GET /recomendacion de la API expuesta.

Dicha función tiene como objetivo procesar una solicitud para obtener una recomendación

de ubicación de heladeras en base a los siguientes parámetros pasados por URL:


 1. Calle (string) : nombre de la calle a partir de la cual se quiere comenzar la recomendación
 2. Altura (string): altura de la calle a partir de la cual se quiere comenzar la recomendación
 3. Provincia (string): provincia de la calle ingresada
 4. radio máximo (int): radio máximo para la búsqueda
 5. stock_minimo (int): stock minimo de viandas a trasladar



### Flujo de la función

1. Se extraen los parámetros mediante Query
2. Se consulta al servicio de georeferenciación (georef), pasándole como parámetro calle, altura y provincia
3. Se procesa la ubicación devuelta por este servicio y se la devuelve en formato JSON





# 2023 Administración de Sistemas
### Orquestación de contenedores

Repositorio donde se actualizará el código para la entrega individual de Administración de Sistemas 2023

El funcionamiento de la aplicacion es sencillo, simplemente ejecutando
```docker compose up``` y asegurándonos de que la máquina en la que se ejecuta permite el tráfico HTTP tendremos nuestra aplicación levantada.

Si accedemos a la dirección IP de la máquina, tendremos una pantalla de inicio (placeholder, ya que no va de diseño web el trabajo). En el navbar podremos crearnos una cuenta, lo que registrará nuestro usuario en la base de datos y creará unas claves pública y privada para un monedero de ETH que se va a conectar a la testnet de Sepolia, guardando la clave privada en un contenedor de Hashicorp/vault.

Una vez creada nuestra cuenta, podremos acceder a "MyPage", donde se mostrará el balance en ETH del que disponemos en nuestro monedero, a la vez que la dirección pública de nuestro monedero. Más abajo, se muestran dos campos para enviar ethereum a la dirección indicada, sin tener en cuenta las fees (casi despreciables). Existe un botón de Actualizar por si hemos recibido o enviado una transacción.
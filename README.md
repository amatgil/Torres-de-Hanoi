# Torres de Hanoi, visualitzades


# Com s'utilitza
Les instruccions bàsiques estan al `justfile`. La comanda principal és `just all`, que 
- crea les carpetes de sortida
- executa rust (que et demanarà `n`), generant tots els `*.ppm`
- utilitza ffmpeg per juntar tots els `*.ppm` en un sol `output.mp4`
- disfruta de la animació :)

# Com funciona
Rust obre dos threads:

- Thread 1:
Per cada pixel, calcula quin dels dos colors ha de tindre (de manera
paral·lela). Cada pixel calculat s'envia al thread 2.

-  Thread 2:
Rep els pixels (no en ordre) dels thread 1. Amb aquest:
    - Si el pixel que tenim és el que toca escriure, l'escriu i continua
    - Si no, l'emmagatzema a un binari heap.

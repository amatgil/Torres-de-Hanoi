# Torres de Hanoi, visualitzades


# Com s'utilitza
Les instruccions bàsiques estan al `justfile`. La comanda principal és `just all [n]`, que 
- crea les carpetes de sortida
- executa rust (que et demanarà `n`), generant tots els `*.ppm`
- utilitza ffmpeg per juntar tots els `*.ppm` en un sol `output.mp4`
- disfruta de la animació :)

## Exemple
Amb n = 8 (8 blocks, 255 moviments, ~4 segons a 60fps)
```bash
just all 8 
```
La sortida serà `output.mp4`.

# Com funciona
És el classic programa recursiu: per moure una serie de `n` blocks, movem els `n - 1` blocks a 
la tercera pila, movem la base a la pila destinació, i movem els `n - 1` blocks a la destinació.
(Si `n == 1`, el movem directament).

Cada cop que mou un block, emmagatzema l'estat a un fitxer seqüencialment.

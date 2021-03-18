# ft_linear_regresion
Projet ft_linear_regression en Rust parce que pourquoi pas?
## Compilation
```
cargo build --release
```
## Programme d'entrainement
```
cargo run --release --bin learn
```

## Programme de prédiction
```
cargo run --release --bin predict
```
## Notes
* Aucun argument à passer pour les deux programmes, ils seront ignorés.
* Apparemment Rust ne laisse pas rentrer des nombre négatifs en paramètre et refusera simplement de lancer le programme.
* Les vilains messages d'erreurs lorsque le programme quitte à cause d'une erreur sont intentionnels, Rust utilise un mécanisme de "unwind" qui remonte le programme en sens inverse pour libérer toute la mémoire qui a été allouée avant l'instruction d'interruption, par conséquent il n'y a pas de leaks.
* Ne pas ajouter l'option --release n'est pas un problème cependant les optimisations du compilateur ne seront pas activés et les options de debug seront activées

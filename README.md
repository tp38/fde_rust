# FDE in rust
The goal of the application is to save Nat's activity for FDE. 
It use the same database as FDE in Python and is written in Rust.
For me, it was just an exercice to learn Rust 

## Installation

Just use cargo to build and run fde. You need to indicate an option. On the first run, perhaps you can use -h (--help) :  

> $ cargo run - --help

```
th@6po:~/Code/Rust/fde$ cargo run - -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/fde - -h`

        fde est un programe permettant de suivre l'activité de Nat.
        Thierry Probst <thierry.probst@free.fr> v1.0.0 du 23/04/2023
        
commande inconnue! fin du programme ...

        Usage : ./fde [option]
        Options :
        -h
        --help          : cette aide.

        -m=date
        --month=date    : spécifie le mois qui sera affiché. le format de date
        use std::io::{self, Write};                  est dd/mm/yyyy.

        -d=date
        --day=date    : spécifie le jour qui sera affiché en vue de sa création, modification
                        ou suppression. le format de date est dd/mm/yyyy.
        
th@6po:~/Code/Rust/fde$
```

## Usage

### input a day result

Just run cargo run with -d option end use m option to modify the default objet :  

> $ cargo run - -d=24/05/2024

```
th@6po:~/Code/Rust/fde$ cargo run - -d=24/05/2024
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/fde - -d=24/05/2024`

        fde est un programe permettant de suivre l'activité de Nat.
        Thierry Probst <thierry.probst@free.fr> v1.0.0 du 23/04/2023
        
commande inconnue! fin du programme ...
(2024-05-24 : 0 [0/0 'None']) a été créé dans la DB
le Ca sélectionné est : (2024-05-24 : 0 [0/0 'None']), que voulez vous faire ? 
[s]: sauvegarder, [d]: effacer, [m]: modifier, [q]: quitter
m
le Ca concerné est : (2024-05-24 : 0 [0/0 'None'])
	chiffre d'affaire : 210
	nb heures : 8
	dont hsup : 0
	commentaire : c'est un test

le Ca est maintenant : (2024-05-24 : 210 [8/0 'Some("c'est un test")'])
th@6po:~/Code/Rust/fde$
```

### display month result

Just run fde with -m option :  

> $ cargo run - -m=24/05/2024

```
th@6po:~/Code/Rust/fde$ cargo run - -m=24/05/2024
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/fde - -m=24/05/2024`

        fde est un programe permettant de suivre l'activité de Nat.
        Thierry Probst <thierry.probst@free.fr> v1.0.0 du 23/04/2023
        
commande inconnue! fin du programme ...
--- Valeurs pour le mois 05-2024 ---
        CA	 =  363.00
        Hours	 =      16
        HSup	 =       1
        Delta	 = -3058.15
        Prime	 =    0.00
--- Données ------------------------
	2024-05-23 :  153 /  8 (1) => "il ne fait pas beau"
	2024-05-24 :  210 /  8 (0) => "c'est un test"
th@6po:~/Code/Rust/fde$
```


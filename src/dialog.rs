//! # dialog :
//! Ce module définit un Dialog permettant l'interface l'utilisateur.
//! Elle contient des routines d'affichage et des routines de questionnement
//! - author : Thierry Probst <thierry.probst@free.fr>
//! - version : 1.0.0
//! - date : 23/04/2023
use rusqlite::Error;
use std::io::{self, Write};

use crate::AUTHOR;
use crate::VERSION;
use crate::VDATE;
use crate::ca::Ca;
use crate::camonth::CaMonth;


#[derive(Debug)]
pub struct Dialog {}

impl Dialog {
    /// show_welcome : affichage d'un message de bienvenue comprenant un descriptif bref de
    /// l'application ainsi que des informations génériques telles que l'auteur, la version,
    /// la date.
    /// - params : aucun
    /// - return : aucun
    pub fn show_welcome() -> () {
        println!( "
        fde est un programe permettant de suivre l'activité de Nat.
        {} v{} du {}
        ", AUTHOR, VERSION, VDATE );
    }

    /// show_help : affichage d'une page d'aide comprenant l'utilisation de la ligne de commande.
    /// - params : aucun
    /// - return : aucun
    pub fn show_help() -> () {
        let help: &'static str = "
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
        ";
        println!("{help}");
    }

    /// something_goes_wrong : standardisation de l'affichage d'un message d'erreur.
    /// - params :
    ///     - fname -> une chaine spécifiant la fonction dans laquelle l'erreur a eu lieu
    ///     - e -> l'erreur proprement dite
    /// return : aucun
    pub fn something_goes_wrong(fname: &str, e: Error ) -> () {
        println!("Quelque chose s'est mal passé dans {fname}: {e}");
    }

    /// display_curmonth_data : affichage des données relatives à un mois particulier.
    /// - params : v -> un Vecteur contenant des objets Ca pour chacun des jours du mois
    /// - return : aucun
    pub fn display_curmonth_data( v: &Vec<Ca> ) -> () {
        for c in v.iter() {
            match &c.comment {
                Some(x) => println!("\t{} : {:>4} / {:2} ({}) => {:?}", c.date, c.ca, c.hours, c.hsup, x ),
                None => println!("\t{} : {:>4} / {:2} ({})", c.date, c.ca, c.hours, c.hsup ),
            }
        }
    }

    /// show_month : affichage d'un rapport relatif à un mois particulier.
    /// ce rapport contient le chiffre d'affaire global, le nombre d'heures et d'heures supplémentaires
    /// ainsi les valeurs relatives à la prime (Delta par rapport au seuil, montant)
    /// - params : m -> un objet CaMonth contenant toutes les données nécessaires
    /// - return : aucun
    pub fn show_month( m: CaMonth ) -> () {
        let delta = m.ca - 3421.15;
        let mut prime = 0.0;
        if delta > 0.0 { prime = m.ca * 0.02; }
        println!( "--- Valeurs pour le mois {0} ---
        CA\t = {1:>7.2}
        Hours\t = {2:>7}
        HSup\t = {3:>7}
        Delta\t = {4:>7.2}
        Prime\t = {5:>7.2}" ,
            m.day.format("%m-%Y").to_string(),
            m.ca,
            m.hours,
            m.hsup,
            delta,
            prime );
        println!("--- Données ------------------------");
        Dialog::display_curmonth_data( &m.datas );

    }

    /// menu_Ca : affiche un objet Ca, propose un menu pour modifier, sauvegarder, effacer
    /// celui-ci en base de données, et renvoi la fonctionnalité choisie
    /// - params : ca -> une référence sur le Ca à afficher
    /// - return : une String contenant la valeur saisie par l'utilisateur
    pub fn menu_ca( c: &Ca ) -> String {
        let choices = "[s]: sauvegarder, [d]: effacer, [m]: modifier, [q]: quitter";
        println!( "le Ca sélectionné est : {}, que voulez vous faire ? ", c);
        println!( "{choices}" );
        let mut input = String::new();
        std::io::stdin().read_line( &mut input ).unwrap();
        return input.trim().to_string();
    }

    /// dialog_Ca : propose d'enregistrer les différents parametres d'un chiffre d'affaire
    /// - params : ca -> le Ca à modifier
    /// - return : le Ca modifié
    pub fn dialog_ca( mut c: Ca ) -> Ca {
        println!( "le Ca concerné est : {c}" );
        let mut input = String::new();

        print!( "\tchiffre d'affaire : " );
        io::stdout().flush().unwrap();
        std::io::stdin().read_line( &mut input ).unwrap();
        c.ca = input.trim().parse().unwrap();

        print!( "\tnb heures : " );
        io::stdout().flush().unwrap();
        input = "".to_string();
        std::io::stdin().read_line( &mut input ).unwrap();
        c.hours = input.trim().parse().unwrap();

        print!( "\tdont hsup : " );
        io::stdout().flush().unwrap();
        input = "".to_string();
        std::io::stdin().read_line( &mut input ).unwrap();
        c.hsup = input.trim().parse().unwrap();

        print!( "\tcommentaire : " );
        io::stdout().flush().unwrap();
        input = "".to_string();
        std::io::stdin().read_line( &mut input ).unwrap();
        let m = input.trim().to_string();
        if m.len() == 0 { c.comment = None; } else { c.comment = Some(m); }

        println!( "\nle Ca est maintenant : {c}" );

        c
    }

}

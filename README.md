# Traitement de fichiers log

Cette application Rust traite des fichiers log en appliquant une expression régulière pour extraire et afficher des informations spécifiques de chaque ligne du log. Les informations extraites incluent les horodatages, les messages du log et les codes de statut HTTP.

## Fonctionnalités

- **Lecture de fichiers** : Ouvre et lit un fichier log ligne par ligne.
- **Correspondance Regex** : Utilise une expression régulière pour identifier et extraire des parties de chaque ligne du log.
- **Affichage des informations** : Affiche l'horodatage, le message et le code de statut pour chaque ligne correspondante.

## Prérequis

- Langage de programmation Rust
- Cargo (le gestionnaire de paquets de Rust)
- Crate `regex` pour les opérations d'expression régulière

## Installation et exécution

1. **Installer Rust et Cargo** :

   - Vous pouvez télécharger Rust et Cargo sur [https://rust-lang.org](https://rust-lang.org).

2. **Ajouter la crate `regex`** :

   - Ajoutez `regex` à vos dépendances dans `Cargo.toml` :
     ```toml
     [dependencies]
     regex = "1.5.4"
     ```

3. **Exécuter le programme** :
   - Placez le fichier log que vous souhaitez traiter à la racine de votre répertoire de projet, ou modifiez le chemin du fichier dans le code source.
   - Utilisez la commande suivante pour exécuter le programme :
     ```bash
     cargo run
     ```

## Configuration

- **Fichier Log** : Le chemin par défaut du fichier log est défini sur `/test_log.log`. Vous pouvez changer ce chemin dans le code source selon vos besoins.

## Format de log attendu

Le programme attend que les entrées de log soient dans le format suivant :

```bash
[AAAA-MM-JJTHH:MM:SS] "Message du log" [Code de Statut]
```

## Gestion des erreurs

Le programme gère les erreurs liées à l'ouverture de fichiers et à la lecture de lignes, et il fournit un retour lorsque une ligne ne correspond pas au format attendu.

## Licence

Ce projet est sous licence MIT.

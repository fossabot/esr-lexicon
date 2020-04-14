<div align="center">

# ESR lexicon

[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![Build Status](https://travis-ci.com/eonm-abes/esr-lexicon.svg?branch=master)](https://travis-ci.com/eonm-abes/esr-lexicon)
[![Coverage Status](https://coveralls.io/repos/github/eonm-abes/esr-lexicon/badge.svg?branch=master)](https://coveralls.io/github/eonm-abes/esr-lexicon?branch=master)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)]()
[![dependency status](https://deps.rs/repo/github/eonm/zotero/status.svg)](https://deps.rs/repo/github/eonm-abes/esr-lexicon)
[![Linux](https://img.shields.io/badge/%E2%80%8B-linux-yellow.svg?logo=linux&style=flat&logoColor=white)](https://github.com/eonm-abes/esr-lexicon/releases/latest/download/esr-lexicon)
[![OSX](https://img.shields.io/badge/%E2%80%8B-osx-lightgrey.svg?logo=apple&style=flat&logoColor=white)](https://github.com/eonm-abes/esr-lexicon/releases/latest/download/esr-lexicon-osx)

</div>

ESR lexicon permet de constituer des dictionnaires d'entités nommées relatifs aux structures de recherche françaises :

* Établissements de l'ESR (libellés, sigles)
* Laboratoires de recherche (libellés, sigles)
* Autres structures de recherche (libellés, sigles)
* Écoles doctorales
* Noms et prénoms des chercheurs
* Noms des unités de recherche
* Noms des départements de recherche

ESR lexicon récupère ses informations depuis [HAL](https://api.archives-ouvertes.fr/search) et [ScanR](https://scanr.enseignementsup-recherche.gouv.fr).

## Téléchargement

ESR lexicon est précompilé pour Linux et Mac.

* Dernière version pour [Linux](https://github.com/eonm-abes/esr-lexicon/releases/latest/download/esr-lexicon)
* Dernière version pour [OSX](https://github.com/eonm-abes/esr-lexicon/releases/latest/download/esr-lexicon-osx)

## Usage

```
$ ./esr-lexicon -h

ESR lexicon 0.2.0
Mathis EON. <eon@abes.fr>
Build NER dictionaries

USAGE:
    esr-lexicon [FLAGS] --config <FILE> --file <FILE> --jq <EXPRESSION> --output <FILE> --url <URL>

FLAGS:
    -h, --help       Prints help information
    -s, --silent     Silent output
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>      Sets a custom config file
    -f, --file <FILE>        Input file
    -j, --jq <EXPRESSION>    Expression used for parsing data
    -o, --output <FILE>      Output file
    -u, --url <URL>          Input URL
```

__En utilisant un fichier de configuration__

```sh
./esr-lexicon -c config.json
```

__En ligne de commande exclusivement__

```sh
./esr-lexicon -o scanr.structure.recherche -j '.["facet_groups"][0]["facets"]|map(.["name"])' -u "https://data.enseignementsup-recherche.gouv.fr/api/records/1.0/search/?dataset=fr-esr-repertoire-national-structures-recherche&rows=0&facet=libelle"
```

### Utilisation des expressions jq

ESR lexicon utilise des [expressions jq](https://stedolan.github.io/jq/manual/) pour extraire les données json.

### Utilisation de la pagination SolR

ERS lexicon suit automatiquement les curseurs SolR pour les requêtes utilisant la [pagination](https://lucene.apache.org/solr/guide/6_6/pagination-of-results.html#fetching-a-large-number-of-sorted-results-cursors) avec `cursorMark=*`.

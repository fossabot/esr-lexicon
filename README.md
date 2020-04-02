# ESR lexicon

ESR lexicon permet de constituer des dictionnaires d'entitées nommées relative aux structures de recherche françaises :

* Etablissements de l'ESR (libélés, sigles)
* Laboratoires de recherche (libélés, sigles)
* Autres structure de recherches (libélés, sigles)
* Écoles doctorales
* Nom et prénom des chercheurs

ESR lexicon récupère ses informations depuis [HAL](https://api.archives-ouvertes.fr/search) et [ScanR](https://scanr.enseignementsup-recherche.gouv.fr).

## Usage

__En utilisant un fichier de configuration__

```sh
./esr-lexicon -c config.json
```

__En ligne de commande exclusivement__

```sh
./esr-lexicon -o scanr.structure.recherche -j '.["facet_groups"][0]["facets"]|map(.["name"])' -u "https://data.enseignementsup-recherche.gouv.fr/api/records/1.0/search/?dataset=fr-esr-repertoire-national-structures-recherche&rows=0&facet=libelle"
```

### Utilisation des expressions jq

ESR lexicon utilise des [expressions jq](https://stedolan.github.io/jq/manual/) pour extraire les données json. Pour constituer les dictionnaires ESR lexicon attend que l'expression jq utilisée retourne un tableau ou une chaine de cractères.

# Journal des versions — NiTriTe Agent

## 1.4.0

- NiTriTe 8.222.0 : démarrage 8 fois plus rapide (tableau de bord en ~8 s au lieu de plus d’une minute). Partages réseau 134 s → 0,7 s, pare-feu ~20 s → 0,2 s, historique système 9,5 s → 0,3 s ; 8 s d’attente maximum par module au chargement.
- Barre d’état : pourcentages arrondis.

## 1.3.0

- Le panneau restait bloqué sur « Chargement… » avec un carré gris : le navigateur refusait le script qui masque l’écran de démarrage (empreinte CSP calculée sur un fichier en fins de ligne Windows). Corrigé.
- Écran de démarrage : le logo NiTriTe remplace le carré gris.

## 1.2.0

- Runtime Visual C++ lié en statique : l’exe démarre sur un Windows neuf sans Visual C++ Redistributable (plus d’erreur « VCRUNTIME140.dll introuvable »).

## 1.1.0

- Cookie de session `HttpOnly; SameSite=Strict` : les autres onglets, les favoris et F5 fonctionnent sans jeton dans l'URL.
- Content-Security-Policy stricte (script d'amorce autorisé par son empreinte), anti-clickjacking, `no-referrer`, COOP/CORP.
- Compression gzip/Brotli de l'interface.
- Menu **Agent** dans le panneau : état, versions, onglets connectés, journal, nouvelle version disponible, arrêt de l'agent.
- Option `--app` : panneau dans une fenêtre d'application Edge.
- Vérification des nouvelles versions au démarrage (`--no-update-check` pour la désactiver).
- Journal `%LOCALAPPDATA%\NiTriTe-WebPanel\agent.log`.
- Exécutable : icône NiTriTe, informations de version, DPI PerMonitorV2.
- Test de fumée sur un vrai Windows en CI, release automatique sur tag, Dependabot (dont le sous-module NiTriTe).

## 1.0.0

- Première version : backend complet de NiTriTe (348 commandes) servi au navigateur, interface de NiTriTe compilée telle quelle, jeton de session, contrôles Host/Origin, confirmations d'administration natives, instance unique, arrêt automatique.

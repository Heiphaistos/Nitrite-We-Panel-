//! Lancement de l'agent a l'ouverture de session Windows. Active, l'agent
//! reste en arriere-plan au lieu de s'arreter quand le dernier onglet se ferme.
//!
//! Tache planifiee « a l'ouverture de session, privileges les plus eleves »
//! et non cle `Run` : l'exe exige les droits administrateur, et Windows
//! ignore sans rien dire les programmes a elever inscrits dans `Run`.

const TASK: &str = "NiTriTe Agent";

#[cfg(windows)]
fn schtasks(args: &[&str]) -> std::io::Result<std::process::Output> {
    use std::os::windows::process::CommandExt;
    // Pas de fenetre de console qui clignote : l'agent n'en a pas.
    std::process::Command::new("schtasks").args(args).creation_flags(0x0800_0000).output()
}

#[cfg(windows)]
pub fn enabled() -> bool {
    schtasks(&["/Query", "/TN", TASK]).is_ok_and(|o| o.status.success())
}

/// Inscrit l'exe en cours (a son emplacement actuel : le deplacer ou le
/// supprimer casse le lancement automatique) ou retire l'inscription.
#[cfg(windows)]
pub fn set(on: bool) -> std::io::Result<()> {
    let out = if on {
        let exe = std::env::current_exe()?;
        let run = format!("\"{}\" --no-browser", exe.display());
        schtasks(&["/Create", "/TN", TASK, "/SC", "ONLOGON", "/RL", "HIGHEST", "/TR", &run, "/F"])?
    } else if enabled() {
        schtasks(&["/Delete", "/TN", TASK, "/F"])?
    } else {
        return Ok(());
    };
    if out.status.success() {
        Ok(())
    } else {
        let msg = String::from_utf8_lossy(&out.stderr);
        Err(std::io::Error::other(msg.trim().to_string()))
    }
}

#[cfg(not(windows))]
pub fn enabled() -> bool {
    false
}

#[cfg(not(windows))]
pub fn set(_on: bool) -> std::io::Result<()> {
    Err(std::io::Error::other("lancement automatique : Windows uniquement"))
}

pub struct ContributorInfo {
    pub name: &'static str,
    pub email: &'static str,
}

pub struct ContributorGroup {
    pub name: &'static str,
    pub contributors: &'static [ContributorInfo],
}

pub static LFBE_CONTRIBUTORS_CORE: &[ContributorInfo] = &[ContributorInfo {
    name: "Kamil Machowski",
    email: "machowskikamil@proton.me",
}];
/*
pub static LFBE_CONTRIBUTORS_ACTIVE: &[ContributorInfo] = &[];
pub static LFBE_CONTRIBUTORS_SERVER: &[ContributorInfo] = &[];
pub static LFBE_CONTRIBUTORS_TRANSLATORS_SUPERVISION: &[ContributorInfo] = &[];
pub static LFBE_CONTRIBUTORS_DOCUMENTATION_SUPERVISION: &[ContributorInfo] = &[];
pub static LFBE_CONTRIBUTORS_PREVIOUS: &[ContributorInfo] = &[];
*/

pub static LFBE_CONTRIBUTORS: &[ContributorGroup] = &[
    ContributorGroup {
        name: "Core developers",
        contributors: LFBE_CONTRIBUTORS_CORE,
    }, /*ContributorGroup {
           name: "Active contributors",
           contributors: LFBE_CONTRIBUTORS_ACTIVE,
       },
       ContributorGroup {
           name: "Servers maintained by",
           contributors: LFBE_CONTRIBUTORS_SERVER,
       },
       ContributorGroup {
           name: "Translations supervision",
           contributors: LFBE_CONTRIBUTORS_TRANSLATORS_SUPERVISION,
       },
       ContributorGroup {
           name: "Documentation supervision",
           contributors: LFBE_CONTRIBUTORS_DOCUMENTATION_SUPERVISION,
       },
       ContributorGroup {
           name: "Previous contributors",
           contributors: LFBE_CONTRIBUTORS_PREVIOUS,
       },*/
];

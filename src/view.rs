use qml::*;
use lazysort::*;

use super::Package;

pub fn show(gathered: Vec<Package>) {
    let mut engine = QmlEngine::new();
    let list = form_list(&gathered);
    let qvar: QVariant = list.get_qvar();

    let mut repos = gathered.iter().map(|p| p.group.clone()).collect::<Vec<_>>();
    repos.dedup();
    let mut groups = gathered.iter()
        .flat_map(|p| p.meta.iter())
        .sorted()
        .cloned()
        .collect::<Vec<_>>();
    groups.dedup();
    let qrepos = repos.iter().map(|s| s.clone().into()).collect::<Vec<QVariant>>();
    let qgroups = groups.iter()
        .map(|s| {
            if s == "" {
                "(no group)".into()
            } else {
                s.as_str().into()
            }
        })
        .collect::<Vec<QVariant>>();

    let qpckgs = QPackages::new(Packages {
        vec: gathered,
        list: list,
        repos: repos,
        groups: groups,
    });
    engine.set_property("packages", &qvar);
    engine.set_and_store_property("qpkgs", qpckgs.get_qobj());
    engine.set_and_store_property("repos", qrepos);
    engine.set_and_store_property("groups", qgroups);
    engine.load_data(include_str!("view.qml"));

    engine.exec();
}

Q_LISTMODEL!{
    pub QPackageList {
        name: &str,
        version: &str,
        repo: &str,
        group: &str,
    }
}

fn form_list(gathered: &[Package]) -> QPackageList {
    let mut qalm = QPackageList::new();
    qalm.set_data(package_to_qvar(gathered, |_| true));
    qalm
}

pub struct Packages {
    vec: Vec<Package>,
    list: QPackageList,
    repos: Vec<String>,
    groups: Vec<String>,
}

fn package_to_qvar<P>(vec: &[Package], filter: P) -> Vec<(&str, &str, &str, &str)>
    where P: FnMut(&&Package) -> bool
{
    vec.into_iter()
        .filter(filter)
        .map(|pkg| {
            let meta = match pkg.meta.first() {
                Some(k) => k,
                None => "",
            };
            (pkg.name.as_str(), pkg.version.as_str(), pkg.group.as_str(), meta)
        })
        .collect()
}

impl Packages {
    fn request_update_repo(&mut self, r: i32) {
        if r == -1 {
            self.list.set_data(package_to_qvar(&self.vec, |_| true))
        } else {
            let vec = package_to_qvar(&self.vec, |pkg| pkg.group == self.repos[r as usize]);
            self.list.set_data(vec);
        }
    }

    fn request_update_group(&mut self, r: i32) {
        if r == -1 {
            self.list.set_data(package_to_qvar(&self.vec, |_| true))
        } else {
            let vec = package_to_qvar(&self.vec, |pkg| pkg.meta.contains(&self.groups[r as usize]));
            self.list.set_data(vec);
        }
    }
}

Q_OBJECT!(
    pub Packages as QPackages {
        signals:

        slots:
        fn request_update_repo(r: i32);
        fn request_update_group(r: i32);

        properties:
    }
);

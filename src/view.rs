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
        chosen_repo: -1,
        chosen_group: -1,
        selected: SelectedPackages::new(),
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
        name: String,
        version: String,
        repo: String,
        group: String,
        selected: bool,
    }
}

fn form_list(gathered: &[Package]) -> QPackageList {
    let mut qalm = QPackageList::new();
    qalm.set_data(filter_for_qml(gathered, |_| true, None));
    qalm
}

pub struct Packages {
    vec: Vec<Package>,
    list: QPackageList,
    repos: Vec<String>,
    groups: Vec<String>,
    chosen_repo: i32,
    chosen_group: i32,
    selected: SelectedPackages,
}

pub struct SelectedPackages {
    vec: Vec<Package>,
}

fn filter_for_qml<P>(vec: &[Package],
                     filter: P,
                     selecteds: Option<&SelectedPackages>)
                     -> Vec<(String, String, String, String, bool)>
    where P: FnMut(&&Package) -> bool
{
    vec.into_iter()
        .filter(filter)
        .map(|pkg| {
            let meta = match pkg.meta.first() {
                Some(k) => k.clone(),
                None => "".into(),
            };
            let selected = if let Some(selecteds) = selecteds {
                selecteds.vec.contains(pkg)
            } else {
                false
            };
            (pkg.name.clone(), pkg.version.clone(), pkg.group.clone(), meta, selected)
        })
        .collect()
}

Q_OBJECT!(
    pub Packages as QPackages {
        signals:
            fn notify_packages_changed(text: String);
        slots:
            fn request_update_repo(r: i32);
            fn request_update_group(r: i32);
            fn add_package(i: i32);
            fn remove_package(i: i32);
        properties:
    }
);

impl QPackages {
    fn request_update_repo(&mut self, r: i32) -> Option<&QVariant> {
        self.chosen_repo = r;
        self.decide_and_update();
        None
    }

    fn request_update_group(&mut self, r: i32) -> Option<&QVariant> {
        self.chosen_group = r;
        self.decide_and_update();
        None
    }

    fn decide_and_update(&mut self) {
        let data = {
            let selected = Some(&self.selected);
            match (self.chosen_repo, self.chosen_group) {
                (-1, -1) => filter_for_qml(&self.vec, |_| true, selected),
                (-1, group) => {
                    filter_for_qml(&self.vec,
                                   |pkg| pkg.meta.contains(&self.groups[group as usize]),
                                   selected)
                }
                (repo, -1) => {
                    filter_for_qml(&self.vec,
                                   |pkg| pkg.group == self.repos[repo as usize],
                                   selected)
                }
                (repo, group) => {
                    filter_for_qml(&self.vec,
                                   |pkg| {
                                       pkg.group == self.repos[repo as usize] &&
                                       pkg.meta.contains(&self.groups[group as usize])
                                   },
                                   selected)
                }
            }
        };
        self.list.set_data(data);
    }

    fn add_package(&mut self, index: i32) -> Option<&QVariant> {
        let pkg_in_list = &self.list.view_data()[index as usize];
        self.list.change_line(index as usize,
                              pkg_in_list.0.clone(),
                              pkg_in_list.1.clone(),
                              pkg_in_list.2.clone(),
                              pkg_in_list.3.clone(),
                              true);
        let pkg_name = &pkg_in_list.0;
        let pkg = self.vec.iter().find(|pkg| pkg_name == &pkg.name).unwrap().clone();
        self.selected.add_package(pkg);
        self.notify_packages_changed(self.selected.get_text());
        None
    }

    fn remove_package(&mut self, index: i32) -> Option<&QVariant> {
        let pkg_in_list = &self.list.view_data()[index as usize];
        self.list.change_line(index as usize,
                              pkg_in_list.0.clone(),
                              pkg_in_list.1.clone(),
                              pkg_in_list.2.clone(),
                              pkg_in_list.3.clone(),
                              false);
        let pkg_name = &pkg_in_list.0;
        let pkg = self.vec.iter().find(|pkg| pkg_name == &pkg.name).unwrap().clone();
        self.selected.remove_package(pkg);
        self.notify_packages_changed(self.selected.get_text());
        None
    }
}

impl SelectedPackages {
    fn new() -> Self {
        SelectedPackages { vec: Vec::new() }
    }

    fn add_package(&mut self, package: Package) {
        self.vec.push(package);
    }

    fn remove_package(&mut self, package: Package) {
        self.vec.retain(|p| p.name != package.name);
    }

    fn get_text(&self) -> String {
        self.vec.iter().map(|p| &p.name as &str).collect::<Vec<&str>>().join(" ")
    }
}

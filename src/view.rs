use qml::*;
use lazysort::*;

use super::Package;

pub fn show(gathered: Vec<Package>) {
    let mut engine = QmlEngine::new();
    let list = form_list(&gathered);
    let qvar: QVariant = list.get_qvar();
    let mut repos = gathered.iter().map(|p| p.group.clone()).collect::<Vec<_>>();
    repos.dedup();
    let qrepos = repos.iter().map(|s| s.clone().into()).collect::<Vec<QVariant>>();

    let qpckgs = QPackages::new(Packages {
        vec: gathered,
        list: list,
        repos: repos,
    });
    engine.set_property("packages", &qvar);
    engine.set_and_store_property("qpkgs", qpckgs.get_qobj());
    engine.set_and_store_property("repos", qrepos);
    engine.load_data(include_str!("view.qml"));

    engine.exec();
}

fn form_list(gathered: &[Package]) -> Box<QListModel<'static>> {
    let mut qalm = QListModel::new(&["name", "version", "repo", "group"]);
    for pkg in gathered {
        // should be an array
        let meta = match pkg.meta.first() {
            Some(k) => k.as_str().into(),
            None => "".into(),
        };
        qalm.insert_row(vec![QVariant::from(pkg.name.as_str()),
                             pkg.version.as_str().into(),
                             pkg.group.as_str().into(),
                             meta]
            .into_iter());
    }
    qalm
}

pub struct Packages {
    vec: Vec<Package>,
    list: Box<QListModel<'static>>,
    repos: Vec<String>,
}

impl Packages {
    fn request_update_repo(&mut self, r: i32) {
        println!("r: {}", r);
        if r == -1 {
            self.list.set_data(self.vec
                .iter()
                .map(|pkg| {
                    let meta = match pkg.meta.first() {
                        Some(k) => k.as_str().into(),
                        None => "".into(),
                    };
                    vec![QVariant::from(pkg.name.as_str()),
                         pkg.version.as_str().into(),
                         pkg.group.as_str().into(),
                         meta]
                })
                .collect())
        } else {
            let vec = self.vec
                .iter()
                .filter(|pkg| pkg.group == self.repos[r as usize])
                .map(|pkg| {
                    let meta = match pkg.meta.first() {
                        Some(k) => k.as_str().into(),
                        None => "".into(),
                    };
                    vec![QVariant::from(pkg.name.as_str()),
                         pkg.version.as_str().into(),
                         pkg.group.as_str().into(),
                         meta]
                })
                .collect();
            self.list.set_data(vec);
        }
    }

    fn request_update_group(&mut self, _: i32) {
        self.list.set_data(Vec::new());
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

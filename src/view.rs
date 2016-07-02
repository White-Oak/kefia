use qml::*;

use super::Package;

pub fn show(gathered: &[Package]) {
    let engine = QmlEngine::new();
    let list = form_list(gathered);
    let qvar: QVariant = list.get_qvar();

    engine.set_property("packages", &qvar);
    engine.load_data(include_str!("view.qml"));

    engine.exec();
}

fn form_list(gathered: &[Package]) -> Box<QListModel<'static>> {
    let mut qalm = QListModel::new(&["name", "version", "repo", "group"]);
    for pkg in gathered {
        // should be an array
        let meta = match pkg.meta.first() {
            Some(&k) => k.into(),
            None => "".into(),
        };
        qalm.insert_row(vec![QVariant::from(pkg.name), pkg.version.into(), pkg.group.into(), meta]
            .into_iter());
    }
    qalm
}

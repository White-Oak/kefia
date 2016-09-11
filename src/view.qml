import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 1.3

ApplicationWindow {
  id: app
  visible: true
  title: "Kefia"
  property real margin: 10
  minimumWidth: total.width
  maximumWidth: total.width
  width: total.width
  minimumHeight: 600
  height: 600
  x: 400
  y: 100
  Item {
    anchors.fill: parent
    anchors.margins: app.margin
    GridLayout{
      id: total
      columns: 2
      rows: 2
      columnSpacing: 10
      rowSpacing: 10
      height: parent.height
      RowLayout{
        id: allGroupsLayout
        Layout.minimumHeight: repoGroup.height
        Layout.maximumHeight: repoGroup.height
        Layout.column: 1
        Layout.row: 1
        GroupBox {
          title: "Repository"
          id: repoGroup

          GridLayout {
            columns: 2
            rows: 2
            RadioButton {
              id: allRepos
              text: " All"
              checked: true
              Layout.columnSpan: 2
              onClicked: {
                repoCB.enabled = false
                someRepos.checked = false
                qpkgs.request_update_repo(-1)
              }
            }
            RadioButton {
              id: someRepos
              checked: false
              onClicked: {
                repoCB.enabled = true
                allRepos.checked = false
                qpkgs.request_update_repo(repoCB.currentIndex)
              }
            }
            ComboBox {
              id: repoCB
              Layout.minimumWidth: 200
              Layout.maximumWidth: 200
              Layout.preferredWidth: 200
              anchors.left: someRepos.right
              anchors.verticalCenter: someRepos.verticalCenter
              enabled: false
              model: repos
              Component.onCompleted: activated.connect(qpkgs.request_update_repo)
            }
          }
        }
        GroupBox {
          title: "Group"

          GridLayout {
            columns: 2
            rows: 2

            RadioButton {
              id: allGroups
              text: " All"
              checked: true
              Layout.columnSpan: 2
              onClicked: {
                groupCB.enabled = false
                someGroups.checked = false
                qpkgs.request_update_group(-1)
              }
            }
            RadioButton {
              id: someGroups
              checked: false
              onClicked: {
                groupCB.enabled = true
                allGroups.checked = false
                qpkgs.request_update_group(groupCB.currentIndex)
              }
            }
            ComboBox {
              id: groupCB
              Layout.minimumWidth: 200
              Layout.maximumWidth: 200
              Layout.preferredWidth: 200
              anchors.left: someGroups.right
              anchors.verticalCenter: someGroups.verticalCenter
              enabled: false
              model: groups
              Component.onCompleted: activated.connect(qpkgs.request_update_group)
            }
          }
        }
      }

      ScrollView {
        id: mainScrollView
        Layout.alignment: Qt.AlignBottom
        Layout.fillHeight: true
        Layout.fillWidth: true
        Layout.column: 1
        Layout.row: 2
        ListView {
          id: mainList
          model: packages
          delegate:
            Rectangle {
              width: mainList.width
              height: childrenRect.height
              RowLayout {
                x: mainList.x + 5
                width: mainList.width - 10
                Text {
                  id: text
                  text: name
                }
                Text {
                  anchors.right: parent.right
                  text: " (" + version + ")"
                  color: "gray"
                }
              }

              MouseArea {
                  anchors.fill: parent
                  onClicked: {
                    if (parent.color == app.color) {
                      parent.color = "lightskyblue"
                      qpkgs.add_package(index)
                    } else {
                      parent.color = app.color
                      qpkgs.remove_package(index)
                    }
                  }
              }
            }

          section.property: "repo"
          section.criteria: ViewSection.FullString
          section.delegate:
            Component {
              Rectangle {
                width: mainList.width
                height: childrenRect.height
                color: "lightsteelblue"

                Text {
                  text: section
                  font.bold: true
                  font.pixelSize: 20
                }
              }
            }
        }
      }

      TextArea {
        id: packagesJoinedTextField
        Layout.minimumWidth: allGroupsLayout.width
        Layout.fillWidth: true
        Layout.column: 2
        Layout.row: 2
        Layout.rowSpan: 2
        Layout.alignment: Qt.AlignBottom
        Component.onCompleted: {
          function setText(text) {
            packagesJoinedTextField.text = text
          }
          qpkgs.notify_packages_changed.connect(setText)
        }
      }
    }
  }
}

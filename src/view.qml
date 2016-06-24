import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 1.3

ApplicationWindow {
  visible: true
  title: "Kefia"
  minimumWidth: 1200
  minimumHeight: 800
  x: 400
  y: 100
  ColumnLayout{
    RowLayout{
      GroupBox {
        title: "Repository"
        id: repoGroup

        ColumnLayout {
          RadioButton {
            id: allRepos
            text: " All"
            checked: true
            Layout.minimumWidth: implicitWidth + repoCB.width
            onClicked: {
              repoCB.enabled = false
              someRepos.checked = false
            }
          }
          RadioButton {
            id: someRepos
            checked: false
            Layout.minimumHeight: implicitHeight + repoCB.height
            onClicked: {
              repoCB.enabled = true
              allRepos.checked = false
            }
            ComboBox {
              id: repoCB
              width: 200
              anchors.left: someRepos.right
              anchors.verticalCenter: someRepos.verticalCenter
              enabled: false
              model: [ "core", "extra", "community" ]
            }
          }
        }
      }
      GroupBox {
        title: "Group"

        ColumnLayout {
          RadioButton {
            id: allGroups
            text: " All"
            checked: true
            Layout.minimumWidth: implicitWidth + groupCB.width
            onClicked: {
              groupCB.enabled = false
              someGroups.checked = false
            }
          }
          RadioButton {
            id: someGroups
            checked: false
            Layout.minimumHeight: implicitHeight + groupCB.height
            onClicked: {
              groupCB.enabled = true
              allGroups.checked = false
            }
            ComboBox {
              id: groupCB
              width: 200
              anchors.left: someGroups.right
              anchors.verticalCenter: someGroups.verticalCenter
              enabled: false
              model: [ "base", "base-devel", "plasma" ]
            }
          }
        }
      }
    }

    ListModel {
      id: packages
      ListElement {
        name: "Bill Smith"
        number: "555 3264"
      }
      ListElement {
        name: "John Brown"
        number: "555 8426"
      }
      ListElement {
        name: "Sam Wise"
        number: "555 0473"
      }
    }

    ListView {
      model: packages
      delegate: Text {
        text: name + ": " + number
      }
    }
  }
}

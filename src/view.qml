import QtQuick 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 1.3

ApplicationWindow {
  id: app
  visible: true
  title: "Kefia"
  minimumWidth: repoGroup.width + 40
  minimumHeight: 800
  x: 400
  y: 100
  ColumnLayout{
    anchors.fill: parent
    anchors.margins: 20
    RowLayout{
      Layout.minimumHeight: repoGroup.height
      Layout.maximumHeight: repoGroup.height
      Layout.alignment: Qt.AlignTop
      GroupBox {
        title: "Repository"
        id: repoGroup
        Layout.minimumHeight:height

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
      LISTPKGS
    }

    ListView {
      id: mainList
      Layout.fillWidth: true
      Layout.alignment: Qt.AlignBottom
      Layout.minimumHeight: 800 - repoGroup.height
      model: packages
      delegate:
      RowLayout {
        width: mainList.width
        Text {
          id: text
          Layout.alignment: Qt.AlignLeft
          text: name
        }
        Text {
          Layout.alignment: Qt.AlignRight
          text: " (" + version + ")"
        }
      }
    }
  }
}

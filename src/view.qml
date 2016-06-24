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

  GroupBox {
    title: "Repository"

    ColumnLayout {
        ExclusiveGroup { id: tabPositionGroup }
        RadioButton {
            id: topButton
            text: "All"
            checked: false
            exclusiveGroup: tabPositionGroup
            Layout.minimumWidth: 100
        }
            ComboBox {
                id: repoCB
                width: 200
                x: 500
                model: [ "Banana", "Apple", "Coconut" ]

                property bool checked: true
                property ExclusiveGroup exclusiveGroup: tabPositionGroup
                onExclusiveGroupChanged: {
                    exclusiveGroup.bindCheckable(repoCB)
                    console.log("EX!")
                }
                onActivated: {
                  checkedChanged
                  console.log("Activated!")
                }
            }
    }
}
}

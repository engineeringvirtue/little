import QtQuick 2.12
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.3

import "./components"

ApplicationWindow {
    visible: true
	width: 600
	height: 900
    title: qsTr("LittleWatch")
	id: window

	FontLoader { id: questrial; name: "Questrial"; source: "../../assets/Questrial/Questrial-Regular.ttf" }
	FontLoader { id: roboto; name: "Roboto"; source: "../../assets/Roboto/Roboto-Regular.ttf" }
	FontLoader { id: robotoBold; name: "Roboto Bold"; source: "../../assets/Roboto/Roboto-Bold.ttf" }
	FontLoader { id: robotoItalic; name: "Roboto Italic"; source: "../../assets/Roboto/Roboto-Italic.ttf" }

	StackLayout {
		anchors.horizontalCenter: parent.horizontalCenter
		anchors.verticalCenter: parent.verticalCenter

		currentIndex: {
			if (watchInterface.connected) {
				1
			} else {
				0
			}
		}

		Pairing {}
		Configuration {}
	}

	Statuses {
		statuses: [watchInterface.status]
	}
}

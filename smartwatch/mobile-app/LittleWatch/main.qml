import QtQuick 2.12
import QtQuick.Controls 2.5

import "./components"

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: qsTr("LittleWatch")
	id: window

	FontLoader { id: questrial; name: "Questrial"; source: "../../assets/Questrial/Questrial-Regular.ttf" }
	FontLoader { id: roboto; name: "Roboto"; source: "../../assets/Roboto/Roboto-Regular.ttf" }
	FontLoader { id: robotoBold; name: "Roboto Bold"; source: "../../assets/Roboto/Roboto-Bold.ttf" }
	FontLoader { id: robotoItalic; name: "Roboto Italic"; source: "../../assets/Roboto/Roboto-Italic.ttf" }

	Loader {
		anchors.fill: parent
		source: watchInterface.connected ? "Configuraiton.qml" : "Pairing.qml"
	}

	Statuses {
		statuses: [watchInterface.status]
	}
}

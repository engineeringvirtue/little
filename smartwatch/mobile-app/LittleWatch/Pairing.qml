import QtQuick 2.12
import "./components"

Column {
	spacing: 30

	Text {
		anchors.horizontalCenter: parent.horizontalCenter
		text: "LittleWatch"
		font.pointSize: 32
		font.family: "Questrial"
	}

	Image {
		anchors.horizontalCenter: parent.horizontalCenter
		width: window.height / 2
		height: window.height / 2
		mipmap: true // big res for all you 4k phones or whatever
		source: "../../assets/logo or something.png"
	}

	BodyText {
		anchors.horizontalCenter: parent.horizontalCenter
		text: watchInterface.status.loading ? "Loading..." : "Not connected. Ensure your watch is powered on and paired."
	}
}

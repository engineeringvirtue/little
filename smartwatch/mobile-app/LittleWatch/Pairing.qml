import QtQuick 2.12
import "./components"

Column {
	anchors.horizontalCenter: parent.horizontalCenter
	anchors.verticalCenter: parent.verticalCenter

	spacing: 30

	Text {
		anchors.horizontalCenter: parent.horizontalCenter
		anchors.top: parent.top + parent.height / 3
		text: "LittleWatch"
		font.pointSize: 32
		font.family: "Questrial"
	}

	Image {
		anchors.horizontalCenter: parent.horizontalCenter
		width: window.height / 2
		height: window.height / 2
		mipmap: true //big res for all you 4k phones or whatever
		source: "../../assets/logo or something.png"
	}

	BodyText {
		anchors.horizontalCenter: parent.horizontalCenter
		text: watchInterface.status.loading ? "Loading..." : "Please make sure your watch is paired."
	}
}

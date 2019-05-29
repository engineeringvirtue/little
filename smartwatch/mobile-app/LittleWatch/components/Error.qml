import QtQuick 2.12

Rectangle {
	x: 0; y: 0
	width: window.width
	height: 40

	color: '#FF6F72'
	border.width: 1
	border.color: 'white'

	Image {
		x: 0; y: 0
		width: parent.height - 1
		height: parent.height - 1
		mipmap: true
		source: "../../../assets/erricon.png"
	}

	BodyText {
		anchors.verticalCenter: parent.verticalCenter
		padding: 10
		leftPadding: parent.height
		text: modelData.error.toString()
		color: 'white'
	}
}

import QtQuick 2.12
import QtQuick.Controls 2.5

import "./widgets"

Column {
	Image {
		height: parent.height / 3
	}

	SwipeView {
		id: view

		currentIndex: 0

		Core {}
	}
}

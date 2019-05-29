import QtQuick 2.12
import com.little.watch 1.0

Column {
	property list<Status> statuses;

	Repeater {
		model: statuses

		Loader {
			source: { modelData.errored ? "./Error.qml" : "" }
		}
	}
}

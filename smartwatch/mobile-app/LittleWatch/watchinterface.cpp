#include "watchinterface.h"

int WatchInterface::getSize(Widget widget) {
	switch (widget) {
	case (Core):
		return int(sizeof(CoreData));
	}
}

void WatchInterface::send(Widget widget, void* data)
{
	QByteArray buffer;
	buffer.resize(2);

	buffer[0] = 1;
	buffer[1] = widget;

	char* data_bytes = reinterpret_cast<char*>(&data);
	buffer.append(QByteArray(data_bytes, getSize(widget)));

	write(buffer);
}

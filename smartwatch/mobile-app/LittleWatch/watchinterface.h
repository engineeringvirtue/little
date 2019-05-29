#ifndef WATCHINTERFACE_H
#define WATCHINTERFACE_H

#include <QObject>
#include <QIODevice>

#include <bluetoothio.h>
#include <status.h>

typedef struct {
	unsigned char weather;
	unsigned short int date;
	unsigned char hour;
	unsigned char minute;
} CoreData;

typedef enum: char {
	Core
} Widget;

class WatchInterface: public PlatformBluetooth
{
	Q_OBJECT
public:
	WatchInterface() {}

	static int getSize(Widget widget);

	void send(Widget widget, void* data);
	void* get(Widget widget);
};

#endif // WATCHINTERFACE_H

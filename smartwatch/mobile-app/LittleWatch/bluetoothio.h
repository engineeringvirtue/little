#ifndef BLUETOOTHIO_H
#define BLUETOOTHIO_H

#include <QTcpServer>
#include <QTcpSocket>
#include <status.h>

class AbstractBluetoothIO: public QIODevice
{
	Q_OBJECT
public:
	Status* status = new Status;
	Q_PROPERTY(Status* status MEMBER status CONSTANT)

	virtual bool isConnected() = 0;
	Q_PROPERTY(bool connected READ isConnected NOTIFY connected)

signals:
	void connected(bool connected);
	void statusChanged();

protected:
	virtual qint64 writeData(const char *data, qint64 maxSize) = 0;
	virtual qint64 readData(char *data, qint64 maxSize) = 0;

private:
	bool m_connected = false;
};

#ifdef QT_DEBUG

class DebugTcpServer: public QTcpServer {
	Q_OBJECT
public:
	DebugTcpServer() {}

protected:
	void incomingConnection(qintptr socketDescriptor);

signals:
	void newDescriptor(qintptr socketDescriptor);
};

class DebugBluetoothIO: public AbstractBluetoothIO
{
	Q_OBJECT
public:
	DebugBluetoothIO();

	bool isConnected();
	QTcpSocket connection;

protected:
	qint64 writeData(const char *data, qint64 maxSize);
	qint64 readData(char *data, qint64 maxSize);

	DebugTcpServer server;
};

typedef DebugBluetoothIO PlatformBluetooth;

#endif

#endif // BLUETOOTHIO_H

#include "bluetoothio.h"

#ifdef QT_DEBUG
DebugBluetoothIO::DebugBluetoothIO() {
	connect(&server, &DebugTcpServer::newDescriptor, this, [=](qintptr descriptor) {
		this->connection.setSocketDescriptor(descriptor);
		connect(&connection, &QAbstractSocket::disconnected, this, [=]() {
			status->clearError();
			emit connected(false);
		});

		connect(&connection, QOverload<QAbstractSocket::SocketError>::of(&QAbstractSocket::error), this->status, &Status::error);

		emit connected(true);
	});

	connect(&server, &QTcpServer::acceptError, this->status, &Status::error);

	if (server.listen(QHostAddress::LocalHost, 8085)) {
		status->loaded();
	} else {
		status->error(server.errorString());
	}
}

bool DebugBluetoothIO::isConnected() {
	return connection.state() == QAbstractSocket::SocketState::ConnectedState;
}

void DebugTcpServer::incomingConnection(qintptr socketDescriptor) {
	emit newDescriptor(socketDescriptor);
}

qint64 DebugBluetoothIO::writeData(const char *data, qint64 maxSize) {
	return connection.write(data, maxSize);
}

qint64 DebugBluetoothIO::readData(char *data, qint64 maxSize) {
	return connection.read(data, maxSize);
}

#endif

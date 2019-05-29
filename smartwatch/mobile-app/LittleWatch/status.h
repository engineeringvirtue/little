#ifndef STATUS_H
#define STATUS_H

#include <QQuickItem>

class Status : public QObject
{
	Q_OBJECT
public:
	Status() {}

	QVariant getError() const { return m_error; }
	bool loading() const { return m_loading; }
	bool didError() const { return m_errored; }

	void loaded() {
		m_loading = false; //one-way switch? for now, i guess
	}

	void clearError() {
		m_errored = false;
		emit erroredChanged();
	}

	Q_PROPERTY(QVariant error MEMBER m_error READ getError CONSTANT) //these arent really constants, but errored should take care of notifying if the actual error has changed
	Q_PROPERTY(bool loading MEMBER m_loading READ loading NOTIFY loadingChanged)
	Q_PROPERTY(bool errored MEMBER m_errored READ didError NOTIFY erroredChanged)

signals:
	void erroredChanged();
	void loadingChanged();

public slots:
	void error(QVariant error) {
		m_error = error;
		m_errored = true;

		emit erroredChanged();
	}

private:
	QVariant m_error;
	bool m_loading = true;
	bool m_errored = false;
};

#endif // STATUS_H

package dbusserver

import (
	"fmt"

	"github.com/abergmeier/illuminationd/internal/server"
	"github.com/godbus/dbus/v5"
	"github.com/godbus/dbus/v5/introspect"
)

const intro = `
<node>
	<interface name="org.illuminationd.IlluminationDaemon.Devices">
		<method name="List">
			<arg direction="out" type="at"/>
		</method>
	</interface>
	<interface name="org.illuminationd.IlluminationDaemon.Settings">
		<method name="Get">
			<arg direction="in" type="t"/>
			<arg direction="out" type="(yvv)"/>
		</method>
		<method name="Set">
			<arg direction="in" type="t"/>
			<arg direction="in" type="(yvv)"/>
		</method>
	</interface>` + introspect.IntrospectDataString + `</node>`

type devices struct {
	internal server.Devices
}

func (ds devices) List() ([]server.Device, *dbus.Error) {
	dl, err := ds.internal.List()
	if err != nil {
		return nil, dbus.NewError("Listing failed", []interface{}{
			err,
		})
	}

	return dl, nil
}

type settings struct {
	internal server.Settings
}

func (ss settings) Get(d server.Device) (server.Values, *dbus.Error) {
	vs, err := ss.internal.GetValues(d)
	if err != nil {
		return server.Values{}, dbus.NewError("Get failed", []interface{}{
			err,
		})
	}
	return vs, nil
}

func (ss settings) Set(d server.Device, vs server.Values) *dbus.Error {
	err := ss.internal.SetValues(d, vs)
	if err != nil {
		return dbus.NewError("Set failed", []interface{}{
			err,
		})
	}
	return nil
}

func Serve(d server.Devices, s server.Settings) error {
	conn, err := dbus.ConnectSessionBus()
	if err != nil {
		panic(err)
	}
	defer conn.Close()

	dbusS := devices{
		internal: d,
	}
	settingsS := settings{
		internal: s,
	}

	mustExport(conn, dbusS, "/org/illuminationd/IlluminationDaemon", "org.illuminationd.IlluminationDaemon.Devices")
	mustExport(conn, settingsS, "/org/illuminationd/IlluminationDaemon", "org.illuminationd.IlluminationDaemon.Settings")
	mustExport(conn, introspect.Introspectable(intro), "/org/illuminationd/IlluminationDaemon",
		"org.freedesktop.DBus.Introspectable")

	reply, err := conn.RequestName("org.illuminationd.IlluminationDaemon",
		dbus.NameFlagDoNotQueue)
	if err != nil {
		return fmt.Errorf("method RequestName failed: %w", err)
	}
	if reply != dbus.RequestNameReplyPrimaryOwner {
		return fmt.Errorf("name `org.illuminationd.IlluminationDaemon` already taken")
	}

	fmt.Println("Listening on org.illuminationd.IlluminationDaemon.Devices / /org/illuminationd/IlluminationDaemon ...")
	fmt.Println("Listening on org.illuminationd.IlluminationDaemon.Settings / /org/illuminationd/IlluminationDaemon ...")
	select {}
}

func mustExport(conn *dbus.Conn, v interface{}, path dbus.ObjectPath, iface string) {
	err := conn.Export(v, path, iface)
	if err != nil {
		panic(err)
	}
}

package main

import (
	"github.com/abergmeier/illuminationd/internal/dbusserver"
	"github.com/abergmeier/illuminationd/internal/demo"
	"github.com/abergmeier/illuminationd/internal/server"
	"github.com/godbus/dbus/v5"
)

func main() {

	data := map[server.Device]server.Values{
		0: {
			On:               true,
			Brightness:       dbus.MakeVariant(uint16(354)),
			ColorTemperature: dbus.MakeVariant(uint16(34)),
		},
	}

	ds := demo.NewDevices(data)
	ss := demo.NewSettings(data)

	dbusserver.Serve(ds, ss)
}

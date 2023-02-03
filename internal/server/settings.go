package server

import "github.com/godbus/dbus/v5"

type Values struct {
	On               bool
	Brightness       dbus.Variant
	ColorTemperature dbus.Variant
}

type Settings interface {
	GetValues(device Device) (Values, error)
	SetValues(device Device, v Values) error
}

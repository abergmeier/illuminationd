package demo

import (
	"github.com/abergmeier/illuminationd/internal/server"
)

type Devices struct {
	data map[server.Device]server.Values
}

func NewDevices(data map[server.Device]server.Values) *Devices {
	return &Devices{
		data: data,
	}
}

func (ds *Devices) List() ([]server.Device, error) {
	var d []server.Device
	for k := range ds.data {
		d = append(d, k)
	}
	return d, nil
}

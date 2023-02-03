package demo

import (
	"fmt"

	"github.com/abergmeier/illuminationd/internal/server"
)

type Settings struct {
	data map[server.Device]server.Values
}

func NewSettings(data map[server.Device]server.Values) *Settings {
	return &Settings{
		data: data,
	}
}

func (s *Settings) GetValues(device server.Device) (server.Values, error) {
	vs, ok := s.data[device]
	if !ok {
		return server.Values{}, fmt.Errorf("device not found")
	}
	return vs, nil
}

func (s *Settings) SetValues(device server.Device, vs server.Values) error {
	s.data[device] = vs
	return nil
}

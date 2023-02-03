package server

type Device uint64

type Devices interface {
	List() ([]Device, error)
}

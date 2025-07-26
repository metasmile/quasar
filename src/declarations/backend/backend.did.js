export const idlFactory = ({ IDL }) => {
  const SupplyDistribution = IDL.Record({
    'date' : IDL.Text,
    'supply' : IDL.Float64,
    'percentage' : IDL.Float64,
  });
  const Result = IDL.Variant({ 'Ok' : SupplyDistribution, 'Err' : IDL.Text });
  const HttpHeader = IDL.Record({ 'value' : IDL.Text, 'name' : IDL.Text });
  const HttpResponse = IDL.Record({
    'status' : IDL.Nat,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HttpHeader),
  });
  return IDL.Service({
    'fetch_supply_data' : IDL.Func([IDL.Text], [Result], []),
    'get_api_key' : IDL.Func([], [IDL.Text], ['query']),
    'get_last_response' : IDL.Func([], [IDL.Text], ['query']),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'set_api_key' : IDL.Func([IDL.Text], [], []),
    'transform_response' : IDL.Func(
        [IDL.Tuple(IDL.Vec(IDL.Nat8), HttpResponse)],
        [HttpResponse],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };

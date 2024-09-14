import 'package:flutter/material.dart';

class PrimaryButton extends StatelessWidget
{
  final VoidCallback onPressed;
  final String text;

  //Constructor
  const PrimaryButton
  ({
    super.key,
    required this.onPressed,
    this.text = ''
  });
  
  @override
  Widget build(BuildContext context) 
  {
    return ElevatedButton
    (
      onPressed: onPressed,
      child: Text(text)
    );
  }
  
  // @override
  // State<StatefulWidget> createState() => _PrimaryButtonState();
}

// class _PrimaryButtonState extends State<PrimaryButton>
// {

//   WidgetStateProperty<Color> _buttonColor = WidgetStateProperty.resolveWith((states) => {
//     if (states.contains(MaterialState.hovered))
//     {
//        Colors.cyan
//     },
//      Colors.blue
//   });

//   void _onHover(bool param)
//   {
//     setState(() 
//     {
//       _buttonColor = Colors.cyan;
//     });
//   }

//   @override
//   Widget build(BuildContext context) 
//   {
//     return ElevatedButton
//     (
//       onPressed: widget.onPressed,
//       onHover: _onHover,
//       style: ButtonStyle
//       (
//         backgroundColor: _buttonColor,
//       ),
//       child: Text(widget.text),
//     );
//   }
// }

class ColumContent extends StatelessWidget
{

  final VoidCallback onIncrement;
  final VoidCallback onDecrement;
  final String text;

  //Constructor
  const ColumContent
  ({
    super.key,
    required this.onIncrement,
    required this.onDecrement,
    this.text = ''

  });

  @override
  Widget build(BuildContext context) 
  {
    return Row
    (
      mainAxisAlignment: MainAxisAlignment.center,
      children: <Widget>
      [
        PrimaryButton
        (
          onPressed: onIncrement,
        ),
        const SizedBox(width: 20),
        ElevatedButton
        (
          onPressed: onDecrement,
          child: Text(text),
        ),
      ],
    );
  }
  
}